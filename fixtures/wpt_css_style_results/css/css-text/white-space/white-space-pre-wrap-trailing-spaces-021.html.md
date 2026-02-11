# css/css-text/white-space/white-space-pre-wrap-trailing-spaces-021.html

```json
{
  "format_version": 3,
  "file": "css/css-text/white-space/white-space-pre-wrap-trailing-spaces-021.html"
}
```

## style[0]

```css

  div
    {
      border: black solid 2px;
      font-family: monospace;
      font-size: 32px;
      line-height: 1.25; /* computes to 40px */
      margin-bottom: 0.25em;
      width: 16ch;
    }

  span
    {
      background-color: yellow;
    }

  div#test
    {
      overflow: auto;
      white-space: pre-wrap;
    }

  div#reference
    {
      white-space: normal;
    }
  
```

```json
{
  "errors": 0,
  "messages": [],
  "warnings": 0
}
```
