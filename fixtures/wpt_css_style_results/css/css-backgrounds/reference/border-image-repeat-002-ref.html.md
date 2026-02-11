# css/css-backgrounds/reference/border-image-repeat-002-ref.html

```json
{
  "format_version": 3,
  "file": "css/css-backgrounds/reference/border-image-repeat-002-ref.html"
}
```

## style[0]

```css

  table
    {
      border-spacing: 0px;
      margin: 70px;
      table-layout: fixed;
    }

  td
    {
      height: 40px;
      padding: 0px;
    }

  td.corner
    {
      background-image: url("../support/new-red-diamond-27x27.png");
      background-size: cover;
    }

  td#first-row-second-cell , td#third-row-second-cell , td.second-row
    {
      background-image: url("../support/blue-diamond-27x27.png");
      background-size: 34px 40px;
    }

  td.second-row
    {
      background-size: 40px 34px;
      height: 102px;
    }
  
```

```json
{
  "errors": 2,
  "messages": [
    {
      "message": "Invalid value for property “background-size”.",
      "severity": "Error"
    },
    {
      "message": "Invalid value for property “background-size”.",
      "severity": "Error"
    }
  ],
  "warnings": 0
}
```
