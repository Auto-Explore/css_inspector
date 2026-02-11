# css/css-ui/nav-up-005.html

```json
{
  "format_version": 3,
  "file": "css/css-ui/nav-up-005.html"
}
```

## style[0]

```css

    div > a {
        float: up;
        margin-up: 1em;
        margin-right: 1em;
    }

    div {
        clear: up;
    }

    #parent {
        nav-up: #finish;
    }

    #intermediate {
        nav-up: #end;
    }
```

```json
{
  "errors": 2,
  "messages": [
    {
      "message": "Invalid value for property “float”.",
      "severity": "Error"
    },
    {
      "message": "Unknown property “margin-up”.",
      "severity": "Error"
    }
  ],
  "warnings": 0
}
```
