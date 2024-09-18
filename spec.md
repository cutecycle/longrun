

# StateStore 
StateStore interface holds program state, and program state is defined by "the set of every persistent variable." it should be compatible with any system that can store a key and a value, including but not limited to rdbms, kv stores, local filesystem, blob storage, memory, etc.f

## Requirements

### Preferred
* Eventual consistency 
### Mandatory


# Variable 



```json
{
    
}
```


# Task 
```json

{
    //immutable
    "Name": "",
    "TaskId": "",
    "StateId": "",
    "StartTime": "", 
    "Version": 0

    //Mutable only upon completion
    "EndTime": "",
    "Status": 200
    
}
```


## Language requirements

### Mandatory
* Lambdas or function pointers
* Interface or Class-level default implementations
### Preferred
* Result types
* 