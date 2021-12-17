# Chronos
Time tracking CLI app written in Rust

## Functionalities
- Track time for project
- Get summary for time spent on project
- Set timers for working on a project
- Export entries

## In-depth functionalities:
- Create a project
  - `chronos project create --name "ProjectName" (--description "Description") (--start "now"/"DD-MM-YYYY-HH-MM") (--end "DD-MM-YYYY-HH-MM")`
- Rename a project
  - `chronos project rename --old-name "ProjectOldName" --new-name "ProjectNewName"`
- Delete project
  - `chronos project delete --name "ProjectName"`
- Add time to a project (total time)
  - `chronos time add --project "ProjectName" --start "now"/"DD-MM-YYYY-HH-MM" (--end "now"/"DD-MM-YYYY-HH-MM") (--amount minutes) (--comment "Comment")`
- Remove time from a project
  - `chronos time remove --project "ProjectName" --id id_of_entry`
- Edit time for a project
  - `chronos time edit --project "ProjectName" --id id_of_entry (--start "now"/"DD-MM-YYYY-HH-MM") (--end "now"/"DD-MM-YYYY-HH-MM") (--amount minutes) (--comment "Comment")`
- Start tracking time on a project
  - `chronos start --project "ProjectName" (--comment "Comment")`
- Stop tracking time on a project and add the elapsed time to the project
  - `chronos stop --project "ProjectName" (--comment "Comment")`
- Get summary for time spent on all projects
  - `chronos summary`
- Get summary for time spent on a certain project (Wildcard matching ?)
  - `chronos summary --project "ProjectName"`
- Export all entries (in JSON)
  - `chronos export`
- Export all entries for a project
  - `chronos export --project "ProjectName"`

