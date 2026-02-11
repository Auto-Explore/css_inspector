# css/css-paint-api/style-before-pseudo.https.html

```json
{
  "format_version": 3,
  "file": "css/css-paint-api/style-before-pseudo.https.html"
}
```

## style[0]

```css

div {
    margin-left: 3px;
}

div::before {
    width: 100px;
    height: 100px;
    content: 'foo';
    color: rgba(0, 0, 0, 0);

    background-image: paint(geometry);
    margin-left: 2px;
    --foo:bar;
}
```

```json
{
  "errors": 0,
  "messages": [],
  "warnings": 0
}
```
