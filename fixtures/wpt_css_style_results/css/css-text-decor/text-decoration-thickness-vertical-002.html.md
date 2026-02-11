# css/css-text-decor/text-decoration-thickness-vertical-002.html

```json
{
  "format_version": 3,
  "file": "css/css-text-decor/text-decoration-thickness-vertical-002.html"
}
```

## style[0]

```css

        #box{
            font: 20px/1 Ahem;
            background-color: red;
            height: 4em;
            width: 1em;
            overflow: hidden;
        }
        /*
         * The underline is placed to the right of the text
         * at the left edge of the #box scrollpart. Its
         * text-decoration should grow "out" away from the
         * text, covering up the scrollport. We will use a
         * 1.2 em-wide decoration line to be sure the
         * scrollport is still fully covered if there are
         * small differences in the placement of the line.
         */
        #text{
            color: transparent;
            writing-mode: vertical-lr;
            position: relative;
            right: 1.1em;
            text-decoration: green underline;
            text-decoration-skip-ink: none;
            text-decoration-thickness: 1.2em;
            text-underline-position: from-font right;
        }
    
```

```json
{
  "errors": 2,
  "messages": [
    {
      "message": "Invalid value for property “text-decoration”.",
      "severity": "Error"
    },
    {
      "message": "Invalid value for property “text-underline-position”.",
      "severity": "Error"
    }
  ],
  "warnings": 0
}
```
