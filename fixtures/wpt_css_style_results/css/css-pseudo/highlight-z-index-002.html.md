# css/css-pseudo/highlight-z-index-002.html

```json
{
  "format_version": 3,
  "file": "css/css-pseudo/highlight-z-index-002.html"
}
```

## style[0]

```css

  div
    {
      font-size: 48px;
      line-height: 1.25; /* computes to 60px */
    }

  div#abs-positioned-sibling
    {
      background-color: yellow;
      color: green;
      position: absolute;
    }

  div#test::selection
    {
      background-color: red;
      color: yellow;
    }
  
```

```json
{
  "errors": 0,
  "messages": [],
  "warnings": 0
}
```
