# css/css-text-decor/text-decoration-thickness-vertical-001.html

```json
{
  "format_version": 3,
  "file": "css/css-text-decor/text-decoration-thickness-vertical-001.html"
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
         * The underline is placed to the left of the text
         * at the right edge of the #box scrollpart. It is
         * 1.3 em away so that the entire text-decoration covers
         * the vertical box. Its text-decoration should grow "out"
         * away from the text, covering up the scrollport. We
         * will use 1.5 em-wide decoration line to be sure the
         * scrollport is still fully covered if there are small
         * differences in the placement of the line.
         */
        #text{
            color: transparent;
            writing-mode: vertical-rl;
            position: relative;
            left: 1.3em;
            text-decoration: green underline;
            text-decoration-skip-ink: none;
            text-decoration-thickness: 1.5em;
        }
    
```

```json
{
  "errors": 1,
  "messages": [
    {
      "message": "Invalid value for property “text-decoration”.",
      "severity": "Error"
    }
  ],
  "warnings": 0
}
```
