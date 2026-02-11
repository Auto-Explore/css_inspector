# css/cssom/change-rule-with-layers-crash.html

```json
{
  "format_version": 3,
  "file": "css/cssom/change-rule-with-layers-crash.html"
}
```

## style[0]

```css

.x {
  transition: color 0.5s;
  @layer warning {
    :first-child { }
    :last-child { }
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
