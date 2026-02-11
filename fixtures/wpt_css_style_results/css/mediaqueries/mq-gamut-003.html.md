# css/mediaqueries/mq-gamut-003.html

```json
{
  "format_version": 3,
  "file": "css/mediaqueries/mq-gamut-003.html"
}
```

## style[0]

```css

div {
    width: 100px;
    height: 100px;
    background-color: green;
}

@media (color-gamut: dci-p3),
       not (color-gamut: dci-p3)
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
