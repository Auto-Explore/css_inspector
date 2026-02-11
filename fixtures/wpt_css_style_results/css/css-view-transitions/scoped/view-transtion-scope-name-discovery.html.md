# css/css-view-transitions/scoped/view-transtion-scope-name-discovery.html

```json
{
  "format_version": 3,
  "file": "css/css-view-transitions/scoped/view-transtion-scope-name-discovery.html"
}
```

## style[0]

```css

  #outer {
    view-transition-name: outer;
  }

  #inner {
    view-transition-name: inner;
  }

  .scoped {
    view-transition-scope: auto;
  }
```

```json
{
  "errors": 3,
  "messages": [
    {
      "message": "Unknown property “view-transition-name”.",
      "severity": "Error"
    },
    {
      "message": "Unknown property “view-transition-name”.",
      "severity": "Error"
    },
    {
      "message": "Unknown property “view-transition-scope”.",
      "severity": "Error"
    }
  ],
  "warnings": 0
}
```
