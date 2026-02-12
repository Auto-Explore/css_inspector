# css/CSS2/css1/c71-fwd-parsing-000.xht

```json
{
  "format_version": 3,
  "file": "css/CSS2/css1/c71-fwd-parsing-000.xht"
}
```

## style[0]

```css
<![CDATA[
   p.one {color: green; rotation: 70deg;}
   p.oneb {color: green;}
   p.oneb {color: invalidValue;}
   div.twopc { background: white url(support/swatch-red.png); color: green; }
   p.two {background-color: inherit;}
   p.eight {COLOR: GREEN;}
   p.twentya {rotation-code: "}"; color: green;}
   p.twentyb {rotation-code: "\"}\""; color: green;}
   p.twentyonea {rotation-code: '}'; color: green;}
   p.twentyoneb {rotation-code: '\'}\''; color: green;}
   p.twentytwo {
    type-display: @threedee {rotation-code: '}';};
    color: green;
    }
  ]]>
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
