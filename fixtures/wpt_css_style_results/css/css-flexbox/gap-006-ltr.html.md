# css/css-flexbox/gap-006-ltr.html

```json
{
  "format_version": 3,
  "file": "css/css-flexbox/gap-006-ltr.html"
}
```

## style[0]

```css

  section {
    background-color: green;
    block-size: 100px;
    inline-size: 200px;
    display: inline-flex;
    flex-wrap: wrap;
    gap: 20px;
  }
  section > div{
    background-color: grey;
    color: white;
    block-size: 20px;
  }
  #bp {
    inline-size: 120px;
  }
  #ww {
    inline-size: 130px;
  }
  #s, #f {
    inline-size: 40px;
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
