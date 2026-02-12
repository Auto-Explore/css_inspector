# css/css-shadow/keyframes-006.html

```json
{
  "format_version": 3,
  "file": "css/css-shadow/keyframes-006.html"
}
```

## style[0]

```css

        @keyframes myanim {
          from { background: red; }
          to { background: green; }
        }
        :host::before, :host::after {
          content: "";
          display: block;
          width: 100px;
          height: 100px;
          background: blue;
          animation: myanim 10s infinite step-end;
        }
      
```

```json
{
  "errors": 0,
  "messages": [],
  "warnings": 0
}
```
