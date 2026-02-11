# css/css-inline/firefox-bug-1901624-crash.html

```json
{
  "format_version": 3,
  "file": "css/css-inline/firefox-bug-1901624-crash.html"
}
```

## style[0]

```css

      :root {
        --sidebar-width: 370px;
      }

      *,
      ::after,
      ::before {
        box-sizing: inherit;
      }
      #sidebar {
        max-width: var(--sidebar-width);
      }
      #codebox pre {
        white-space: pre-wrap;
      }
      #codebox pre .copy-btn-displacement {
        float: right;
        height: 40px;
        width: 40px;
      }
      #codebox #codebox-answer li {
        hyphens: auto;
      }
      html {
        font-size: 20px;
        box-sizing: border-box;
      }
    
```

```json
{
  "errors": 0,
  "messages": [],
  "warnings": 0
}
```
