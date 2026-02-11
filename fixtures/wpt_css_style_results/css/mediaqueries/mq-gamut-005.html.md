# css/mediaqueries/mq-gamut-005.html

```json
{
  "format_version": 3,
  "file": "css/mediaqueries/mq-gamut-005.html"
}
```

## style[0]

```css

div {
    width: 100px;
    height: 100px;
    background-color: green;
}

@media (color-gamut: rec-2020),
       not (color-gamut: rec-2020)
{
    div { background-color: red; }
}
```

```json
{
  "errors": 0,
  "messages": [],
  "warnings": 0
}
```
