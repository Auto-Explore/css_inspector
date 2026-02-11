# css/CSS2/bidi-text/bidi-004-ref.xht

```json
{
  "format_version": 3,
  "file": "css/CSS2/bidi-text/bidi-004-ref.xht"
}
```

## style[0]

```css

   div p { width: 28em; border: solid; margin: 1em; padding: 0.5em; background: #FFFFCC; color: black; font: 1em/1 Ahem; }
   .control { line-height: 3em; }
   .control.start { border-style: solid none solid solid; padding: 0.4em 0 0.4em 1em;  }
   .control.middle { border-style: solid none solid none; padding: 0.4em 0 0.4em 0;  }
   .control.end { border-style: solid solid solid none; padding: 0.4em 1em 0.4em 0;  }
   .a { color: navy; }
   .b { color: orange; }
  
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
