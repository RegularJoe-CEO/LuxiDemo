#!/usr/bin/env python3
# SPDX-FileCopyrightText: 2025 Eric Waller
# SPDX-License-Identifier: LicenseRef-Luxi-Business-1.0

"""
Economics calculator for Luxi Edge energy savings.
Generates CFO-ready summary with multiple scenarios.
"""

import argparse
from pathlib import Path


def calculate_savings(p_mw: float, price: float, f: float, r: float) -> float:
    """
    Calculate annual energy savings.
    
    Formula: Savings($/yr) = P_facility_MW * f * r * 8760 * 1000 * price_per_kWh
    
    Args:
        p_mw: Facility power in MW
        price: Energy price per kWh
        f: Fraction of IT workload touched
        r: Energy reduction on that fraction
    
    Returns:
        Annual savings in dollars
    """
    return p_mw * f * r * 8760 * 1000 * price


def generate_economics_summary(scenarios: list, out_file: Path):
    """Generate economics summary markdown."""
    
    md = []
    md.append("# Luxi Edge Economics Summary")
    md.append("")
    md.append("## Formula")
    md.append("")
    md.append("```")
    md.append("Savings($/yr) = P_facility_MW × f × r × 8760 × 1000 × price_per_kWh")
    md.append("```")
    md.append("")
    md.append("Where:")
    md.append("- **P_facility_MW**: Facility power consumption in megawatts")
    md.append("- **f**: Fraction of IT workload touched by Luxi Edge")
    md.append("- **r**: Energy reduction achieved on that fraction")
    md.append("- **8760**: Hours per year")
    md.append("- **price_per_kWh**: Energy cost per kilowatt-hour")
    md.append("")
    md.append("**Note**: PUE (Power Usage Effectiveness) cancels out if uniform across baseline and optimized scenarios.")
    md.append("")
    
    md.append("## Scenarios")
    md.append("")
    md.append("| Scenario | P (MW) | Price ($/kWh) | f | r | Annual Savings |")
    md.append("|----------|--------|---------------|---|---|----------------|")
    
    for scenario in scenarios:
        name = scenario["name"]
        p_mw = scenario["p_mw"]
        price = scenario["price"]
        f = scenario["f"]
        r = scenario["r"]
        savings = calculate_savings(p_mw, price, f, r)
        
        md.append(f"| {name} | {p_mw} | ${price:.2f} | {f:.2f} | {r:.2f} | ${savings:,.0f} |")
    
    md.append("")
    md.append("## Calculation Details")
    md.append("")
    
    for scenario in scenarios:
        name = scenario["name"]
        p_mw = scenario["p_mw"]
        price = scenario["price"]
        f = scenario["f"]
        r = scenario["r"]
        savings = calculate_savings(p_mw, price, f, r)
        
        md.append(f"**{name}**: {p_mw} × {f} × {r} × 8760 × 1000 × {price} = ${savings:,.0f}/yr")
        md.append("")
    
    md.append("## Interpretation")
    md.append("")
    md.append("These scenarios demonstrate the potential cost savings from deploying Luxi Edge:")
    md.append("")
    md.append("- **S1 (Conservative)**: Small pilot deployment touching 10% of workloads with 30% energy reduction")
    md.append("- **S2 (Moderate)**: Broader deployment touching 20% of workloads with 50% energy reduction")
    md.append("- **S3 (Aggressive)**: Deep deployment touching 30% of workloads with 50% energy reduction")
    md.append("")
    md.append("The actual savings will depend on:")
    md.append("1. Workload characteristics (math-intensive workloads benefit more)")
    md.append("2. Deployment scope (percentage of infrastructure touched)")
    md.append("3. Energy efficiency gains (dependent on baseline vs. SIMD/GPU acceleration)")
    md.append("4. Energy costs (vary by region and provider)")
    md.append("")
    md.append("## ROI Considerations")
    md.append("")
    md.append("Beyond direct energy savings, Luxi Edge provides:")
    md.append("- **Performance improvement**: 13.7× speedup enables higher throughput")
    md.append("- **Infrastructure efficiency**: Race-to-idle reduces active compute time")
    md.append("- **Predictable costs**: Deterministic performance enables capacity planning")
    md.append("- **Carbon reduction**: Lower energy consumption reduces carbon footprint")
    md.append("")
    
    # Write to file
    out_file.parent.mkdir(parents=True, exist_ok=True)
    with open(out_file, "w") as f:
        f.write("\n".join(md))
    
    print(f"✓ Generated {out_file}")


def main():
    parser = argparse.ArgumentParser(description="Generate economics summary for Luxi Edge")
    parser.add_argument("--p_mw", type=float, default=100, help="Facility power in MW")
    parser.add_argument("--price", type=float, default=0.10, help="Energy price per kWh")
    parser.add_argument("--f", type=float, default=0.10, help="Fraction of IT touched")
    parser.add_argument("--r", type=float, default=0.30, help="Energy reduction ratio")
    parser.add_argument("--out", type=Path, required=True, help="Output markdown file")
    
    args = parser.parse_args()
    
    # Define standard scenarios
    scenarios = [
        {
            "name": "S1 (Conservative)",
            "p_mw": 100,
            "price": 0.10,
            "f": 0.10,
            "r": 0.30
        },
        {
            "name": "S2 (Moderate)",
            "p_mw": 100,
            "price": 0.20,
            "f": 0.20,
            "r": 0.50
        },
        {
            "name": "S3 (Aggressive)",
            "p_mw": 50,
            "price": 0.30,
            "f": 0.30,
            "r": 0.50
        }
    ]
    
    # Add custom scenario if provided
    if any([args.p_mw != 100, args.price != 0.10, args.f != 0.10, args.r != 0.30]):
        scenarios.append({
            "name": "Custom",
            "p_mw": args.p_mw,
            "price": args.price,
            "f": args.f,
            "r": args.r
        })
    
    generate_economics_summary(scenarios, args.out)


if __name__ == "__main__":
    main()
