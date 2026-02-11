# css/css-pseudo/highlight-cascade/highlight-cascade-007.html

```json
{
  "format_version": 3,
  "file": "css/css-pseudo/highlight-cascade/highlight-cascade-007.html"
}
```

## style[0]

```css

    main {
        font-size: 12px;
        line-height: 13px;
    }
    main span {
        font-size: 18px;
        line-height: 19px;
    }
    /* * (universal) */::selection {
        font-size: 42px;
        line-height: 43px;
    }
    main .M::selection {
        text-shadow: green 1em 0;
    }
    main .W::selection {
        text-shadow: green 0 1lh;
    }
    /* * (universal) */::selection {
        text-decoration-thickness: 1em;
    }
```

```json
{
  "errors": 2,
  "messages": [
    {
      "message": "Invalid value for property “text-shadow”.",
      "severity": "Error"
    },
    {
      "message": "Invalid value for property “text-shadow”.",
      "severity": "Error"
    }
  ],
  "warnings": 0
}
```
