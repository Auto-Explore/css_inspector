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
  "errors": 1,
  "messages": [
    {
      "message": "Unknown at-rule.",
      "severity": "Error"
    }
  ],
  "warnings": 0
}
```
