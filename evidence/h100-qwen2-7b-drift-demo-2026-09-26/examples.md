# Real examples: visible answer changed under load (default engines)

Rates (all non-reference load conditions): vLLM 0.25.1 default: 12/140, SGLang 0.5.19 default: 11/140. Most prompt-conditions did NOT change visibly; these are the chosen examples.

## vLLM 0.25.1 default - prompt p08 (reasoning), condition `b16_f1` vs `b1`

First differing token index: 3. Max |logprob diff| before divergence: 0.000558.

| alone (reference) | under load |
|---|---|
| ...The second youngest **is Erin.  ⏎  ⏎ Here's the reasoning: ⏎  ⏎ 1. Alice is older than Bob. ⏎ 2. Bob is older than Carol. ⏎ 3. Dave is younger than Carol but older than Erin. ⏎  ⏎ From these statements, we can deduce the following order: ⏎  ⏎ Alice > Bob > Carol > Dave > Erin ⏎  ⏎ So,** | ...The second youngest **person is Erin.  ⏎  ⏎ Here's the reasoning: ⏎  ⏎ 1. Alice is older than Bob. ⏎ 2. Bob is older than Carol. ⏎ 3. Dave is younger than Carol but older than Erin. ⏎  ⏎ From these statements, we can deduce the following order of ages: ⏎  ⏎ Alice > Bob > Carol >** |

Word diff of the full visible answers (`-` alone only, `+` under load only):

```diff
+ person
- order:

Alice
+ order of ages:

Alice
- Erin

So,
+ Erin

Therefore, Erin is
- person is Erin.
+ person.
```

Luxi, same prompt and condition: tokens identical = True, logprob hash identical = True.

## vLLM 0.25.1 default - prompt p07 (reasoning), condition `b16_f1` vs `b1`

First differing token index: 33. Max |logprob diff| before divergence: 0.00763.

| alone (reference) | under load |
|---|---|
| ...To find the arrival time of the train, we need to add the duration of the trip to the departure time. ⏎  ⏎ The departure time is 2:45 **PM. The trip takes 3 hours and 50 minutes. ⏎  ⏎ First, let's convert the duration into minutes for easier calculation: ⏎  ⏎ 3 hours = 3 * 60 minutes = 180 minutes ⏎ 50 minutes = 50 minutes ⏎  ⏎ Total duration in minutes = 180 minutes + 50 minutes** | ...To find the arrival time of the train, we need to add the duration of the trip to the departure time. ⏎  ⏎ The departure time is 2:45 **PM, and the trip takes 3 hours and 50 minutes. ⏎  ⏎ First, let's convert the duration of the trip into minutes for easier calculation: ⏎  ⏎ 3 hours = 3 * 60 minutes = 180 minutes ⏎ 50 minutes = 50 minutes ⏎  ⏎ Total duration in minutes = 180** |

Word diff of the full visible answers (`-` alone only, `+` under load only):

```diff
- PM. The
+ PM, and the
+ of the trip
+ +
- duration,
+ minutes,
- minutes

Adding
+ minutes

So, we add 3 hours to
- hours:
2:45
+ departure time:

2:45
- PM

Adding
+ PM

Now, we add
- minutes:
5:45
+ remaining 50 minutes:

5:45
- PM

So,
+ PM

Therefore,
```

Luxi, same prompt and condition: tokens identical = True, logprob hash identical = True.

## vLLM 0.25.1 default - prompt p17 (writing), condition `b64_f1` vs `b1`

First differing token index: 40. Max |logprob diff| before divergence: 0.00726.

| alone (reference) | under load |
|---|---|
| ...to work from locations of their choice ⏎ - Potential for reduced overhead costs for the company, as it may not need as much physical office space ⏎ - **Ability to access a wider talent pool, as remote work removes geographical barriers ⏎ - Improved work-life balance for employees, as they can better manage their personal and professional responsibilities ⏎ - Increased productivity for some employees, as they may find a more conducive** | ...to work from locations of their choice ⏎ - Potential for reduced overhead costs for the company, as it may not need as much physical office space ⏎ - **Enhanced work-life balance for employees, as they can better manage their personal and professional responsibilities ⏎ - Access to a wider talent pool, as the company can hire from anywhere in the world ⏎ - Increased productivity for some employees, as they may find** |

Word diff of the full visible answers (`-` alone only, `+` under load only):

```diff
- Ability to access a wider talent pool, as remote work removes geographical barriers
- Improved
+ Enhanced
+ Access to a wider talent pool, as the company can hire from anywhere in the world
-
- Opportunity for
+ Environmental benefits, as fewer
+ commuting can lead
- save time and money on commuting

Cons:
-
+ reduced carbon footprint

Cons:
-
- leading to misunderstandings or decreased
+ affecting
- cohesion
-
+ cohesion and collaboration
- Potential for reduced productivity for others, as home distractions can be a significant barrier
-
- managing
+ monitoring employee work hours and ensuring they are meeting deadlines
- Increased cybersecurity risks, as
- teams effectively,
+ workers may use personal devices and networks that are not
- it can be harder to monitor progress and ensure accountability
- Potential for isolation or loneliness among remote workers, especially if they lack a dedicated workspace or social interactions
- Technical issues, such
+ secure
- unreliable internet connectivity or access to necessary software, can hinder productivity
-
+ company-provided infrastructure
-
+ employees who are physically distant
- Technical issues, such as unreliable internet connections or compatibility problems with
- employees
- Increased difficulty in onboarding new employees and integrating them into the company culture
+ work tools, can hinder productivity
```

Luxi, same prompt and condition: tokens identical = True, logprob hash identical = True.
