import argparse, asyncio, os, signal, sys
async def pipe(reader, writer):
    try:
        while True:
            data = await reader.read(65536)
            if not data: break
            writer.write(data)
            await writer.drain()
    except Exception:
        pass
    finally:
        try: writer.close()
        except Exception: pass

async def handle_unix(reader_u, writer_u, host, port):
    try:
        r_t, w_t = await asyncio.open_connection(host, port)
    except Exception as e:
        writer_u.close()
        return
    await asyncio.gather(
        pipe(reader_u, w_t),
        pipe(r_t, writer_u)
    )

async def main():
    ap = argparse.ArgumentParser()
    ap.add_argument('--uds', default='/tmp/erock.sock')
    ap.add_argument('--tcp', default='127.0.0.1:8080')
    args = ap.parse_args()
    host, port = args.tcp.split(':'); port = int(port)
    try:
        if os.path.exists(args.uds): os.remove(args.uds)
    except Exception:
        pass
    loop = asyncio.get_event_loop()
    server = await asyncio.start_unix_server(lambda r,w: handle_unix(r,w,host,port), path=args.uds)
    print(f"UDS proxy listening at {args.uds} -> {host}:{port}", flush=True)
    stop = asyncio.Event()
    for sig in (signal.SIGINT, signal.SIGTERM):
        loop.add_signal_handler(sig, stop.set)
    async with server:
        await stop.wait()
        server.close()
        await server.wait_closed()
        try:
            if os.path.exists(args.uds): os.remove(args.uds)
        except Exception:
            pass
if __name__ == '__main__':
    try:
        asyncio.run(main())
    except KeyboardInterrupt:
        pass
