# ArcDPS Buddy
[ArcDPS](https://deltaconnected.com/arcdps) plugin for [Guild Wars 2](https://guildwars2.com) assisting with combat gameplay.

## Features
- Track skill cast states, durations & hit counts
- Track buff applications to other players & NPCs
- Track breakbar damage hits

## Casts
Displays casted skills and their durations and hit counts.
Duration is colored based on the animation as categorized by ArcDPS (full aftercast, cancelled aftercast, interrupt).
Hit count is colored based on the expected and maximum amount of hits.

You can supply custom skill definitions via `arcdps_buddy_skills.yml`.
The plugin ships with the default definitions in [src/data](./src/data/).
A single entry for a skill looks like this:

```yml
- id: 12345 # id of casted skill.
  hit_ids: [23456, 34567] # optional: additional skill ids to count for hits.
  hits: 5 # optional: number of hits. enables hit tracking. set to 0 to track with unknown hits.
  expected: 4 # optional: number of expected hits. threshold for yellow color. defaults to >= half hits.
  max_duration: 10000 # optional: maximum duration (ms). hits after the duration count towards a new cast.
  minion: true # optional: whether to include hits from own minions.
```

Individual default skill definitions can be overwritten or disabled:

```yml
- id: 12345 # overwrite the default entry entirely
  hits: 5
- id: 12345 # disable the default entry
  enabled: false
```

## Buffs
Displays buffs applied to other players and NPCs.
Includes buff applications from own minions.
Ignores applications from self to self, own minion to self and own minion to same minion.
Tracks
[Quickness](https://wiki.guildwars2.com/wiki/Quickness),
[Alacrity](https://wiki.guildwars2.com/wiki/Alacrity),
Thief [Venom](https://wiki.guildwars2.com/wiki/Venom),
[Moa Stance](https://wiki.guildwars2.com/wiki/Moa_Stance),
[Vulture Stance](https://wiki.guildwars2.com/wiki/Vulture_Stance),
[One Wolf Pack](https://wiki.guildwars2.com/wiki/One_Wolf_Pack) applications.
Target is colored based on being a player or NPC/minion.

## Breakbar
Displays [defiance bar](https://wiki.guildwars2.com/wiki/Defiance_bar) damage hits and their respective skill name, damage amount and target.
Target is colored based on main log target species.
