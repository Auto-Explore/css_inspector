# css/css-cascade/scope-shadow-sharing.html

```json
{
  "format_version": 3,
  "file": "css/css-cascade/scope-shadow-sharing.html"
}
```

## style[0]

```css

    div {
      padding: 3px;
      border: 1px solid;
      background: white;
    }

    @scope {
      div {
        background: red;
      }
    }

    @scope (.explicit-scope) {
      div {
        background: green;
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

## style[1]

```css

      @scope {
        div {
          background: blue;
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
