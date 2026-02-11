# css/mediaqueries/mq-gamut-001.html

```json
{
  "format_version": 3,
  "file": "css/mediaqueries/mq-gamut-001.html"
}
```

## style[0]

```css

div {
    width: 100px;
    height: 100px;
    background-color: red;
}

@media (color-gamut: srgb),
       not (color-gamut: srgb)
{
    div { background-color: green; }
}
```

```json
{
  "errors": 0,
  "messages": [],
  "warnings": 0
}
```
