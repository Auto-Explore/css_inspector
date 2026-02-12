# css/css-text-decor/text-decoration-style-multiple.html

```json
{
  "format_version": 3,
  "file": "css/css-text-decor/text-decoration-style-multiple.html"
}
```

## style[0]

```css

    div {
        color: #aaa;
        font-size: 50px;
        position: relative;
        display: inline-block;
        width: 200px;
        height: 200px;
    }

    div > span { position: absolute; }

    div > span:nth-child(3) { text-decoration: underline solid coral; }
    div > span:nth-child(3) > span { text-decoration: overline dashed skyblue; }
    div > span:nth-child(3) > span  > span { text-decoration: line-through wavy green; }

```

```json
{
  "errors": 0,
  "messages": [],
  "warnings": 0
}
```
