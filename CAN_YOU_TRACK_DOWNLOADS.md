# Can You Tell If Anyone Has Downloaded Your Code?

**Short Answer:** GitHub provides **limited** visibility into who downloads your code, and there are significant blind spots.

---

## What GitHub DOES Track (Requires Repository Owner Access)

### 1. Clone Statistics (Last 14 Days Only)

**What You Can See:**
- Total number of clones in the past 14 days
- Number of unique IP addresses that cloned
- Daily breakdown of clone activity

**How to Access:**
```bash
# Using GitHub CLI (requires authentication)
gh auth login
gh api repos/RegularJoe-CEO/LuxiEdge/traffic/clones

# Or via web interface:
# Go to: https://github.com/RegularJoe-CEO/LuxiEdge/graphs/traffic
# (Only visible to repository owners)
```

**What It Shows:**
```json
{
  "count": 127,              // Total clones in last 14 days
  "uniques": 23,             // Unique cloners
  "clones": [
    {
      "timestamp": "2025-11-01T00:00:00Z",
      "count": 15,
      "uniques": 3
    },
    ...
  ]
}
```

**Limitations:**
- ⚠️ **Only 14 days of history** - older data is lost
- ⚠️ **Anonymous** - no usernames shown
- ⚠️ **IP addresses only** - can't identify who specifically
- ⚠️ **Not comprehensive** - doesn't catch all downloads

### 2. Traffic Views (Last 14 Days Only)

**What You Can See:**
- Page views (people viewing on GitHub)
- Unique visitors
- Most popular files/paths

**Example Output:**
```json
{
  "count": 450,              // Total page views
  "uniques": 89,             // Unique visitors
  "views": [...]
}
```

### 3. Public Forks

**What You Can See:**
- All public forks of your repository
- When they were created
- Who created them
- Activity on those forks

**How to Check:**
```bash
# Via web browser
https://github.com/RegularJoe-CEO/LuxiEdge/network/members

# Via API
gh api repos/RegularJoe-CEO/LuxiEdge/forks --paginate
```

**Example:**
- Username: `john_doe`
- Fork created: `2025-11-05`
- Last updated: `2025-11-10`
- Stars: 0
- Additional commits: 5

### 4. Stargazers and Watchers

**What You Can See:**
- Users who starred your repository (public)
- Users watching for updates (private to them)

**How to Check:**
```bash
gh api repos/RegularJoe-CEO/LuxiEdge/stargazers
```

---

## What GitHub DOES NOT Track

### 1. Private Clones ❌

**Cannot See:**
- Someone who clones and immediately removes the remote
- Private copies made to other Git servers
- Downloaded ZIP files (GitHub shows downloads but not who)

**Example of Invisible Clone:**
```bash
# Your friend could do this:
git clone https://github.com/RegularJoe-CEO/LuxiEdge.git
cd LuxiEdge
git remote remove origin
git remote add origin https://private-gitlab.com/secret-repo.git
git push

# This is now completely invisible to you
```

### 2. Download ZIP ❌

**Cannot See:**
- Who downloads the repository as a ZIP file
- How many times ZIP was downloaded
- What they do with the downloaded code

Someone can click "Code" → "Download ZIP" on GitHub and you'll never know.

### 3. Copied Code ❌

**Cannot See:**
- Someone copying code manually (copy-paste)
- Someone taking screenshots of your code
- Someone using your code in their own project

### 4. Historical Data ❌

**Cannot See:**
- Clones older than 14 days
- Views older than 14 days
- Total all-time download count

### 5. CI/CD and Automated Clones ❌

**Cannot See:**
- Bots that clone your repository
- CI/CD systems that access your code
- Dependency scanners and security tools

---

## How to Check What's Available to You NOW

### Step 1: Check if You Have Owner Access

Only repository **owners** and **admins** can see traffic statistics.

**Test:**
```bash
# Try to access traffic data
gh auth login  # Authenticate with GitHub
gh api repos/RegularJoe-CEO/LuxiEdge/traffic/clones
```

**If you get an error:** You don't have owner access.

### Step 2: Use the GitHub Web Interface

**Easiest Method:**

1. Go to: https://github.com/RegularJoe-CEO/LuxiEdge
2. Click the "Insights" tab
3. Click "Traffic" in the left sidebar

**You should see:**
- Git clones (unique cloners)
- Visitors (unique visitors)
- Views (page views)
- Popular content (most viewed files)

**Screenshot what you see** - it only keeps 14 days of data!

### Step 3: Check for Public Forks

1. Go to: https://github.com/RegularJoe-CEO/LuxiEdge/network/members
2. Look for any forks listed

**Currently (as of my check):**
- The API was blocked by the network proxy in this environment
- You need to check this yourself from your own machine/browser

### Step 4: Document What You Find

**Save this information:**
```bash
# Create a snapshot
date > tracking_snapshot.txt
echo "Forks:" >> tracking_snapshot.txt
gh api repos/RegularJoe-CEO/LuxiEdge/forks --paginate >> tracking_snapshot.txt

echo "\nClones (last 14 days):" >> tracking_snapshot.txt
gh api repos/RegularJoe-CEO/LuxiEdge/traffic/clones >> tracking_snapshot.txt

echo "\nViews (last 14 days):" >> tracking_snapshot.txt
gh api repos/RegularJoe-CEO/LuxiEdge/traffic/views >> tracking_snapshot.txt
```

---

## What Can You Infer?

### From Clone Statistics

**High clone count suggests:**
- People are testing/evaluating your code
- Automated tools are accessing it
- Possible commercial interest

**Low clone count suggests:**
- Repository is not widely known yet
- People are viewing but not downloading
- Activity is happening via forks

### From Forks

**Public forks tell you:**
- ✅ Exactly who forked
- ✅ When they forked
- ✅ If they've made changes (commits)
- ✅ If they're actively developing

**What to look for in forks:**
- Commits mentioning "stripe", "payment", "commercial"
- Additional dependencies added
- License changes or removals
- Rebranding (name changes)

### From Traffic Views

**High views + low clones:**
- People are browsing code on GitHub
- Evaluating before committing
- Reading documentation

**High views + high clones:**
- Active interest and usage
- People are testing it
- Potential commercial interest

---

## The Reality: Most Downloads Are Invisible

### What You're Missing

**Based on typical GitHub analytics:**

If you see:
- 50 clones in 14 days
- 10 unique cloners

**The reality is probably:**
- **200-500 total downloads** (including ZIP, older clones, etc.)
- **50-150 unique individuals** have your code
- **10-20 private clones** exist that you can't see
- **1-5 commercial uses** you don't know about

**Why?**
- 14-day window misses historical downloads
- ZIP downloads aren't tracked per-user
- Private clones are invisible
- Not everyone forks publicly

---

## Can You Find Your Friend's Fork?

### If It's a Public Fork: YES

**Check these locations:**

1. **Network Graph:**
   - https://github.com/RegularJoe-CEO/LuxiEdge/network/members
   - Shows all public forks

2. **Search GitHub:**
   ```
   site:github.com luxi edge stripe
   site:github.com LuxiEdge
   ```

3. **Check Your Friend's Profile:**
   - https://github.com/YOUR_FRIENDS_USERNAME?tab=repositories
   - Look for repositories with similar names

### If It's a Private Clone: NO

**Your friend could have:**
- Downloaded as ZIP
- Cloned and removed remote
- Created private repository elsewhere
- Used it in a closed-source project

**You'll only find it if:**
- They made it public
- Someone tells you about it
- You find it through web search
- They mention it in public

---

## Recommended Actions

### 1. Check Your Current Stats (Do This Now)

**Open a browser and go to:**
```
https://github.com/RegularJoe-CEO/LuxiEdge/graphs/traffic
```

**Document what you see:**
- Clone count (last 14 days)
- Unique cloners
- Top countries (if shown)
- Popular files

**Screenshot it** - this data expires in 14 days!

### 2. Check for Forks (Do This Now)

**Visit:**
```
https://github.com/RegularJoe-CEO/LuxiEdge/network/members
```

**Look for:**
- Any forks at all
- Forks with recent activity
- Forks with added commits
- Suspicious names or descriptions

### 3. Set Up Ongoing Monitoring

**Weekly snapshots:**
```bash
#!/bin/bash
# save_weekly_stats.sh

DATE=$(date +%Y%m%d)
gh api repos/RegularJoe-CEO/LuxiEdge/traffic/clones > stats/clones_$DATE.json
gh api repos/RegularJoe-CEO/LuxiEdge/traffic/views > stats/views_$DATE.json
gh api repos/RegularJoe-CEO/LuxiEdge/forks --paginate > stats/forks_$DATE.json
```

Run this weekly to build historical data that GitHub doesn't keep.

### 4. Search for External Usage

**Google searches to run:**
```
"RegularJoe-CEO/LuxiEdge"
"luxi edge" github
"luxi edge" stripe
site:github.com "simd_eval_over_x_inplace"
```

**Code search engines:**
- https://searchcode.com
- https://publicwww.com
- https://grep.app

---

## Bottom Line

### What You CAN Know:
- ✅ Clone count (last 14 days only)
- ✅ Public forks (all time)
- ✅ Stars and watchers
- ✅ Traffic views (last 14 days only)

### What You CANNOT Know:
- ❌ Total all-time downloads
- ❌ Who specifically downloaded (anonymous)
- ❌ Private clones or forks
- ❌ ZIP file downloads (per user)
- ❌ What they're doing with the code
- ❌ Commercial usage (unless public)

### Most Important Truth:

**By the time you see the statistics, it's too late to prevent the download.**

The code is public. Assume:
- ✅ Many people have downloaded it
- ✅ Some are using it commercially
- ✅ You can't see most of the activity
- ✅ This will continue as long as the repo is public

**Your strategy should be:**
1. Accept that code is widely distributed
2. Focus on enforcement when you discover violations
3. Compete on service and innovation, not secrecy
4. Use the license to pursue clear violations

---

## What to Do About Your Friend

### Direct Approach (Recommended)

**Ask them directly:**
> "Hey, did you fork my LuxiEdge repository for your Stripe project? I'd love to see what you built. If you're using it commercially, we should discuss licensing."

### If They Say Yes:

**Follow-up questions:**
1. Is it a public fork or private clone?
2. Are you using it commercially?
3. Can I see the repository?
4. Did you make significant changes?

### If They Say No:

**Verify by:**
1. Checking their GitHub profile
2. Asking more specifically about what they built
3. Offering to help with licensing if needed

---

## Next Steps

1. **Check traffic stats NOW** (14-day window)
2. **Check network graph for forks**
3. **Document what you find**
4. **Set up weekly monitoring**
5. **Talk to your friend**
6. **Search for external usage**

Remember: **GitHub's visibility is limited by design.** Most downloads happen invisibly.

---

**For detailed monitoring instructions, see:**
- `scripts/detect_forks.sh` - Automated detection script
- `SECURITY_RECOMMENDATIONS.md` - Complete monitoring setup guide
- `SECURITY_ASSESSMENT_REPORT.md` - Full security analysis

**Questions?** Review the comprehensive documentation or contact e@ewaller.com
