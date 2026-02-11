# css/mediaqueries/mq-gamut-002.html

```json
{
  "format_version": 3,
  "file": "css/mediaqueries/mq-gamut-002.html"
}
```

## style[0]

```css

div {
    width: 100px;
    height: 100px;
    background-color: red;
}

@media (color-gamut: p3),
       not (color-gamut: p3)
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
