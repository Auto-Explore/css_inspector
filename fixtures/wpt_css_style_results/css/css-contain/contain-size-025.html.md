# css/css-contain/contain-size-025.html

```json
{
  "format_version": 3,
  "file": "css/css-contain/contain-size-025.html"
}
```

## style[0]

```css

  div.inline-block
    {
      display: inline-block;
    }

  div#blue-test
    {
      background-color: blue;
      contain: size;
      padding: 50px;
    }

  span.block-descendant, img
    {
      display: block;
    }

  div#orange-reference
    {
      background-color: orange;
      height: 100px;
      width: 100px;
    }
  
```

```json
{
  "errors": 1,
  "messages": [
    {
      "message": "Invalid value for property “background-color”.",
      "severity": "Error"
    }
  ],
  "warnings": 0
}
```
