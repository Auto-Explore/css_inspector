# css/css-page/margin-boxes/dimensions-001-print.html

```json
{
  "format_version": 3,
  "file": "css/css-page/margin-boxes/dimensions-001-print.html"
}
```

## style[0]

```css

  @page {
    margin: 100px;
    size: 500px 400px;

    @top-left {
      border: solid;
      text-align: left;
      vertical-align: top;
      width: 20%;
      height: 20%;
      content: "20%";
    }
    @right-middle {
      text-align: left;
      vertical-align: top;
      border: solid;
      width: 70%;
      height: 70%;
      content: "70%";
    }
    @bottom-right {
      text-align: left;
      vertical-align: top;
      border: solid;
      content: "auto";
    }
    @left-bottom {
      text-align: left;
      vertical-align: top;
      border: solid;
      content: "auto";
    }
  }
```

```json
{
  "errors": 1,
  "messages": [
    {
      "message": "Unknown at-rule.",
      "severity": "Error"
    }
  ],
  "warnings": 0
}
```
