# css/mediaqueries/mq-deprecated-001.html

```json
{
  "format_version": 3,
  "file": "css/mediaqueries/mq-deprecated-001.html"
}
```

## style[0]

```css

div {
    width: 100px;
    height: 100px;
    background-color: green;
}

@media
    tty,
    tv,
    projection,
    handheld,
    braille,
    embossed,
    aural,
    speech
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
