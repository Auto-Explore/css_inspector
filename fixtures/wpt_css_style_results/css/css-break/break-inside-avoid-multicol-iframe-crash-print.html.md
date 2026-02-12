# css/css-break/break-inside-avoid-multicol-iframe-crash-print.html

```json
{
  "format_version": 3,
  "file": "css/css-break/break-inside-avoid-multicol-iframe-crash-print.html"
}
```

## style[0]

```css

        body { overflow: hidden; }
    
```

```json
{
  "errors": 0,
  "messages": [],
  "warnings": 0
}
```

## style[1]

```css

        html { overflow: hidden; }
        body {
            /* page width */
            width: 971px;
            height: 810px;
            column-fill: auto;
            column-gap: 80px;

            /* (page width / 2) - column-gap */
            column-width: 405.5px;
            font-size: 16px;
        }

        section.sect2 {
            page-break-after: always;
            -webkit-column-break-after: always;
            break-inside: avoid;
        }
    
```

```json
{
  "errors": 0,
  "messages": [],
  "warnings": 0
}
```
