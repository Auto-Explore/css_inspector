# css/mediaqueries/mq-gamut-004.html

```json
{
  "format_version": 3,
  "file": "css/mediaqueries/mq-gamut-004.html"
}
```

## style[0]

```css

div {
    width: 100px;
    height: 100px;
    background-color: red;
}

@media (color-gamut: rec2020),
       not (color-gamut: rec2020)
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
