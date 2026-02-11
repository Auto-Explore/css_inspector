# css/css-anchor-position/try-tactic-base.html

```json
{
  "format_version": 3,
  "file": "css/css-anchor-position/try-tactic-base.html"
}
```

## style[0]

```css

  #cb {
    position: absolute;
    width: 400px;
    height: 200px;
    border: 1px solid black;
  }
  #target {
    position: absolute;
    /* Does not fit initially */
    width: 150px;
    height: 300px;
    border: 3px solid skyblue;
  }
```

```json
{
  "errors": 1,
  "messages": [
    {
      "message": "Invalid value for property “border”.",
      "severity": "Error"
    }
  ],
  "warnings": 0
}
```
