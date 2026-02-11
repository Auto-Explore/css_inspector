# css/css-flexbox/fieldset-as-container-justify-center.tentative.html

```json
{
  "format_version": 3,
  "file": "css/css-flexbox/fieldset-as-container-justify-center.tentative.html"
}
```

## style[0]

```css

  fieldset {
    display: flex;
    flex-flow: column;
    justify-content: center;
    align-items: center;
    width: 200px;
    min-height: 200px;
    padding: 0px;
    border: 1px solid;
    position: relative;
  }

  .item {
    height: 100px;
    width: 100px;
    background: orange;
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
