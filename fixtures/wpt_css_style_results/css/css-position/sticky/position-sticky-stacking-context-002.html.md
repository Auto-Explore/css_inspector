# css/css-position/sticky/position-sticky-stacking-context-002.html

```json
{
  "format_version": 3,
  "file": "css/css-position/sticky/position-sticky-stacking-context-002.html"
}
```

## style[0]

```css

  div
    {
      height: 100px;
      width: 100px;
    }

  div#overlapped-red
    {
      background-color: red;
      position: absolute;
    }

  div#sticky
    {
      background: linear-gradient(to bottom, green 51%, red 49%);
      position: sticky;
    }

  div#overlap-bottom-half-of-sticky
    {
      background-color: green;
      bottom: 50px;
      height: 50px;
      position: relative;
    }
  
```

```json
{
  "errors": 1,
  "messages": [
    {
      "message": "Invalid value for property “background”.",
      "severity": "Error"
    }
  ],
  "warnings": 0
}
```
