# css/css-ruby/br-clear-all-000.html

```json
{
  "format_version": 3,
  "file": "css/css-ruby/br-clear-all-000.html"
}
```

## style[0]

```css

  #float {
    float: left;
    width: 100px;
    height: 100px;
    background: cyan;
  }
  #container {
    padding-bottom: 50px;
    line-height: 20px;
    background: yellow;
  }
  ruby {
    ruby-position: under;
  }
  ruby > div {
    display: inline-block;
    width: 20px;
    height: 20px;
    background: hotpink;
  }
  rt > div {
    display: inline-block;
    width: 50px;
    height: 50px;
    background: blue;
  }
```

```json
{
  "errors": 2,
  "messages": [
    {
      "message": "Invalid value for property “background”.",
      "severity": "Error"
    },
    {
      "message": "Invalid value for property “background”.",
      "severity": "Error"
    }
  ],
  "warnings": 0
}
```
