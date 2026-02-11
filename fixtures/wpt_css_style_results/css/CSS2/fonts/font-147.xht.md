# css/CSS2/fonts/font-147.xht

```json
{
  "format_version": 3,
  "file": "css/CSS2/fonts/font-147.xht"
}
```

## style[0]

```css

   /* valids */
   .a { font: 10px Ahem Black, Ahem;  }
   .b { font: 10px inherit, Ahem;  }
   .c { font: 10px \" , Ahem; }
  
```

```json
{
  "errors": 2,
  "messages": [
    {
      "message": "Unbalanced braces.",
      "severity": "Error"
    },
    {
      "message": "Invalid value for property “font”.",
      "severity": "Error"
    }
  ],
  "warnings": 0
}
```

## style[1]

```css

   .d { font: 10px 32px, Ahem;  }
   .e { font: 10px -test, Ahem;  }
   /* invalids */
   .f { font: 32px :-), sans-serif;  }
   .g { font: 32px , sans-serif;  }
   .h { font: 32px (), sans-serif;  }
   .i { font: 32px {}, sans-serif;  }
   .j { font: 32px [], sans-serif;  }
   .k { font: 32px a(), sans-serif;  }
   .l { font: 32px a{}, sans-serif;  }
   .m { font: 32px a[], sans-serif;  }
   .n { font: 32px; }
   .o { font: 32px \"", sans-serif;  }
  
```

```json
{
  "errors": 4,
  "messages": [
    {
      "message": "Invalid value for property “font”.",
      "severity": "Error"
    },
    {
      "message": "Missing ':' in declaration.",
      "severity": "Error"
    },
    {
      "message": "Missing ':' in declaration.",
      "severity": "Error"
    },
    {
      "message": "Invalid value for property “font”.",
      "severity": "Error"
    }
  ],
  "warnings": 0
}
```

## style[2]

```css

   .P { font: 32px \\", sans-serif;  }
  
```

```json
{
  "errors": 1,
  "messages": [
    {
      "message": "Unbalanced braces.",
      "severity": "Error"
    }
  ],
  "warnings": 0
}
```

## style[3]

```css

   /* environment */
   * { margin: 0; padding: 0; border: none; line-height: 1; }
   body { margin: 1em; position: relative; }
   .control { display: inline; font: 1em/1 Ahem, sans-serif; background: red; color: white; }
   .overlapper { position: absolute; left: 0; top: 1em; height: 160px; width: 60px; background: green; z-index: 1; -moz-opacity: 90%; }
   .valid { font: 32px sans-serif; color: red; }
   .invalid { font: 10px Ahem; color: red; }
  
```

```json
{
  "errors": 1,
  "messages": [
    {
      "message": "Unknown property “-moz-opacity”.",
      "severity": "Error"
    }
  ],
  "warnings": 0
}
```
