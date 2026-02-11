# css/css-view-transitions/navigation/with-types/at-rule-opt-in-auto-with-types-no-cascade.html

```json
{
  "format_version": 3,
  "file": "css/css-view-transitions/navigation/with-types/at-rule-opt-in-auto-with-types-no-cascade.html"
}
```

## style[0]

```css

  /* This should be ignored, because the following rule has the default "none" types */
  @view-transition {
    types: old-type;
  }

  @view-transition {
    navigation: auto;
  }
```

```json
{
  "errors": 0,
  "messages": [],
  "warnings": 0
}
```
