# css/CSS2/css1/c71-fwd-parsing-002.xht

```json
{
  "format_version": 3,
  "file": "css/CSS2/css1/c71-fwd-parsing-002.xht"
}
```

## style[0]

```css
<![CDATA[
   body { color: green; }
   h1 + p.three {color: red;}
   p.four + h1 {color: red;}
   p.five {background-color: "red";}
   @three-dee {
    @background-lighting {
     azimuth: 30deg;
     elevation: 190deg;
     }
    p.seven { color: red }
    }
   ol:wait {color:red;}
   p.ten:first-child {color: red;}
   ul:lang(fr) {color:red;}
   blockquote[href] {color: red;}
   acronym[href="foo"] {color: red;}
   address[href~="foo"] {color: red;}
   span[lang|="fr"] {color: #f00;}
   @media tty {
    h1 {color: red;}
    p.sixteen {color: red;}
    }
   @three-dee {
    p.seventeen {color: red }
    }
   // UL.nineteenb,
   p.nineteenb {color: red;}
   p.twentythree {text-indent: 0.5in;}
    color: red
   p.twentyfour {color: red;}
  ]]>
```

```json
{
  "errors": 1,
  "messages": [
    {
      "message": "Invalid value for property “background-color”.",
      "severity": "Error"
    }
  ],
  "warnings": 0
}
```
