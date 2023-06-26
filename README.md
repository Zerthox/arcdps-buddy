# ArcDPS Buddy
[ArcDPS](https://deltaconnected.com/arcdps) plugin for [Guild Wars 2](https://guildwars2.com) assisting with combat gameplay.

## Features
- Track skill cast states, durations & hit counts
- Track Quickness & Alacrity applications to other players & NPCs
- Track breakbar damage hits

## Casts
You can supply custom skill definitions via `arcdps_buddy_skills.yml`.
A single entry for a skill looks like this:

```yml
- id: 45717 # id of casted skill.
  hit_id: 42145 # optional: id of hit skill, if different from cast.
  hits: 5 # optional: number of hits. enables hit tracking. set to 0 to track with unknown hits.
  expected: 4 # optional: number of expected hits. threshold for yellow color. defaults to >= half hits.
```

Individual default skill definitions can be overwritten or disabled:

```yml
- id: 12345 # overwrite the default entry entirely
  hits: 5
- id: 12345 # disable the default entry
  enabled: false
```
 