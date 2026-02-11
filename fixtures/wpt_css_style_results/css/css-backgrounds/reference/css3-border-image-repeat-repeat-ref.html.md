# css/css-backgrounds/reference/css3-border-image-repeat-repeat-ref.html

```json
{
  "format_version": 3,
  "file": "css/css-backgrounds/reference/css3-border-image-repeat-repeat-ref.html"
}
```

## style[0]

```css

  table
    {
    border-spacing: 0px;
    table-layout: fixed;
    }

  td
    {
      height: 18px;
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
      background-size: contain;
      background-position: center;
    }

  td.second-row
    {
      height: 108px; /* 6 times 18 == 108 */
    }
  
```

```json
{
  "errors": 0,
  "messages": [],
  "warnings": 0
}
```
