# readme

let's try with a simple example:

```swift reds
LogChannel(n"DEBUG", "hello world");
```

```swift
print("Hello, World!") 
```

```rust
fn main() { println!("hello world"); }
```

```lua
print("Hello World")
```

```cpp
#include <iostream>

int main() {
    std::cout << "Hello World!";
    return 0;
}
```

```yaml
some:
  interesting:
    - property
```

```json
{
  "some": { "interesting": ["property"] }
}
```

```xml
<some>
  <interesting>
    <property />
  </interesting>
</some>
```

contrary to code blocks,
`inline` and ```fenced``` are left untouched.

it should also work when deeply nested:

1. outer:
   1. inner:

      ```swift reds
      GameInstance
        .GetStatusEffectSystem(this.GetGame())
        .ApplyStatusEffect(
          this.GetEntityID(),
          t"BaseStatusEffect.SplinterAddicted",
          this.GetRecordID(),
          this.GetEntityID());
      ```
