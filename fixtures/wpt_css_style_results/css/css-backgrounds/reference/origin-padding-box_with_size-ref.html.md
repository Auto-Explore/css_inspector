# css/css-backgrounds/reference/origin-padding-box_with_size-ref.html

```json
{
  "format_version": 3,
  "file": "css/css-backgrounds/reference/origin-padding-box_with_size-ref.html"
}
```

## style[0]

```css

  body
    {
      height: 568px;
      overflow-y: hidden;
      width: 584px;
    }

  div
    {
      display: inline-block;
      vertical-align: bottom;
    }

  div.image
    {
      height: 288px;
      width: 514px;
    }

  img
    {
      left: 16px;
      position: relative;
      top: 16px;
    }

  div#multiple
    {
      background-image: url('../background-origin/support/yellow-orange-blue-160x160.png');
      background-position: 16px 16px;
      background-size: 241px auto;
      bottom: 288px;
      margin-top: 8px;
      position: relative;
    }

  div.light-blue-border
    {
      border: 16px solid rgba(60, 150, 255, 0.4);
      bottom: 288px;
      height: 256px;
      position: relative;
      width: 482px;
    }
  
```

```json
{
  "errors": 1,
  "messages": [
    {
      "message": "Invalid value for property “background-size”.",
      "severity": "Error"
    }
  ],
  "warnings": 0
}
```
