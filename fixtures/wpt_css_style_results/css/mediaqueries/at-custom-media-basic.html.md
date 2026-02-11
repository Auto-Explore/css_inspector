# css/mediaqueries/at-custom-media-basic.html

```json
{
  "format_version": 3,
  "file": "css/mediaqueries/at-custom-media-basic.html"
}
```

## style[0]

```css

@custom-media --foo (width > 0px);
@media (--foo) {
  :root {
    background: green;
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
