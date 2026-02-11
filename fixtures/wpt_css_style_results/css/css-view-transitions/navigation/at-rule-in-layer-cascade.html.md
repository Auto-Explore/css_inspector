# css/css-view-transitions/navigation/at-rule-in-layer-cascade.html

```json
{
  "format_version": 3,
  "file": "css/css-view-transitions/navigation/at-rule-in-layer-cascade.html"
}
```

## style[0]

```css

@layer inertA, inertB, active;

@layer inertA {
  @view-transition {
    navigation: none;
  }
}
@layer active {
  @view-transition {
    navigation: auto;
  }
}
@layer inertB {
  @view-transition {
    navigation: none;
  }
}
```

```json
{
  "errors": 0,
  "messages": [],
  "warnings": 0
}
```
