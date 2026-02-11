# css/css-backgrounds/border-image-007.html

```json
{
  "format_version": 3,
  "file": "css/css-backgrounds/border-image-007.html"
}
```

## style[0]

```css

  div.outer-parent
    {
      border: red solid 40px;
      border-image-slice: 24;
      border-image-source: url("support/50x50-green.png");
      height: 12px;
      margin-bottom: 16px;
      padding: 2px 4px 6px 8px;
      width: 8px;
    }

  div.inner-child
    {
      background-color: blue;
      height: 100%;
      width: 100%;
    }

  div#first-subtest
    {
      border-image-width: 1px;
    }

  div#second-subtest
    {
      border-image-width: 1px 5px;
    }

  div#third-subtest
    {
      border-image-width: 1px 5px 10px;
    }

  div#fourth-subtest
    {
      border-image-width: 1px 5px 10px 15px;
    }
  
```

```json
{
  "errors": 3,
  "messages": [
    {
      "message": "Invalid value for property “border-image-width”.",
      "severity": "Error"
    },
    {
      "message": "Invalid value for property “border-image-width”.",
      "severity": "Error"
    },
    {
      "message": "Invalid value for property “border-image-width”.",
      "severity": "Error"
    }
  ],
  "warnings": 0
}
```
