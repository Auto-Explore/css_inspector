# css/css-fonts/font-size-adjust-012-ref.html

```json
{
  "format_version": 3,
  "file": "css/css-fonts/font-size-adjust-012-ref.html"
}
```

## style[0]

```css

        @font-face {
            font-family: 'ahem-ex-500';
            src: url('./resources/ahem-ex-500.otf') format('opentype');
        }
        @font-face {
            font-family: 'ahem-ex-250';
            src: url('./resources/ahem-ex-250.otf') format('opentype');
        }
        .wrapper {
            --primary-font: 'ahem-ex-500';
            --secondary-font: 'ahem-ex-250';
        }
        .test {
            font-family: var(--primary-font), var(--secondary-font);
            font-size: 100px;
            line-height: 1;
            color: peru;
            height: 100px;
        }
        .primary-font {
            font-family: var(--primary-font);
        }
        .secondary-font {
            font-family: var(--secondary-font);
        }
        .tall-inline-block {
            display: inline-block;
            height: 100px;
        }
        .description {
            font-family: 'Times New Roman';
            font-size: 14px;
        }
        .main {
            font-size: 16px;
            margin-bottom: 8px;
        }
    
```

```json
{
  "errors": 1,
  "messages": [
    {
      "message": "Invalid value for property “color”.",
      "severity": "Error"
    }
  ],
  "warnings": 0
}
```
