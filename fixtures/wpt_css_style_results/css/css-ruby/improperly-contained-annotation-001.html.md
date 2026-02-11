# css/css-ruby/improperly-contained-annotation-001.html

```json
{
  "format_version": 3,
  "file": "css/css-ruby/improperly-contained-annotation-001.html"
}
```

## style[0]

```css

  ruby
    {
      background-color: lightblue;
      font-size: 40px;
    }

  rbc
    {
      display: ruby-base-container;
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
