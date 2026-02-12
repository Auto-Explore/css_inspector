# css/CSS2/syntax/at-rule-013.xht

```json
{
  "format_version": 3,
  "file": "css/CSS2/syntax/at-rule-013.xht"
}
```

## style[0]

```css
<![CDATA[
    p {
      color: red;
      background: red;
    }
    @media all {
      #semicolon { background: transparent; }
      @foo ] & | # $ % test-token \
       [; # { background: red; } ]
       (; #semicolon { background: red; } } } } )
       '; #semicolon { background: red; } } } }',
       "; #semicolon { background: red; }' } } }"
      ;
      #semicolon { color: green; }
    }
    @media all {
      #block { background: transparent; }
      @foo ] & | # $ % test-token \
       [; #block { background: red; } ]
       (; #block { background: red; } )
       '; #block { background: red; }',
       "; #block { background: red; }'"
       {; #block { background: red; }
          #block { background: red; } }
      #block { color: green; }
    }
    @media all {
      #eob { background: transparent; }
      @import "support/import-red.css"
    }
    #eob {
      color: green;
    }
    @media all {
      #eob-complex { background: transparent; }
      @import "support/import-red.css"
       [; #eob-complex { background: red; } ]
       (; #eob-complex { background: red; } )
       '; #eob-complex { background: red; }',
       "; #eob-complex { background: red; }'"
    }
    #eob-complex {
      color: green;
    }
  ]]>
```

```json
{
  "errors": 3,
  "messages": [
    {
      "message": "Unbalanced braces.",
      "severity": "Error"
    },
    {
      "message": "Unbalanced braces.",
      "severity": "Error"
    },
    {
      "message": "Unbalanced braces.",
      "severity": "Error"
    }
  ],
  "warnings": 0
}
```
