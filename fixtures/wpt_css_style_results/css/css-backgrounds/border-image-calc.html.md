# css/css-backgrounds/border-image-calc.html

```json
{
  "format_version": 3,
  "file": "css/css-backgrounds/border-image-calc.html"
}
```

## style[0]

```css

  #test {
    background-color: red;
    border: 100px solid red;
    border-image-slice: 10;
    border-image-source: url("support/green_color.png");
    border-image-width: 100px calc(100px) calc(100%) calc(50% + 50px);
    height: 0;
    width: 0;
  }
```

```json
{
  "errors": 1,
  "messages": [
    {
      "message": "Invalid value for property “border-image-width”.",
      "severity": "Error"
    }
  ],
  "warnings": 0
}
```
