# css/css-anchor-position/anchor-scope-shadow-names.html

```json
{
  "format_version": 3,
  "file": "css/css-anchor-position/anchor-scope-shadow-names.html"
}
```

## style[0]

```css

    #test_part {
      .host::part(scope) {
        /* This should have no effect, because --a is specified in a different
           tree-scope than the --a inside the shadow. */
        anchor-scope: --a;
      }
    }
  
```

```json
{
  "errors": 2,
  "messages": [
    {
      "message": "Invalid selector.",
      "severity": "Error"
    },
    {
      "message": "Unknown property “anchor-scope”.",
      "severity": "Error"
    }
  ],
  "warnings": 0
}
```

## style[1]

```css

        .anchored {
          background: coral;
          position: absolute;
          top: anchor(bottom, 1px);
          position-anchor: --a;
          width: 5px;
          height: 5px;
        }
        .anchor {
          background: skyblue;
          height: 10px;
          anchor-name: --a;
        }
        .cb {
          position: relative;
          width: 50px;
          height: 50px;
          border: 1px solid black;
        }
        .scope {
          anchor-scope: --a;
        }
      
```

```json
{
  "errors": 5,
  "messages": [
    {
      "message": "Invalid value for property “background”.",
      "severity": "Error"
    },
    {
      "message": "Unknown property “position-anchor”.",
      "severity": "Error"
    },
    {
      "message": "Invalid value for property “background”.",
      "severity": "Error"
    },
    {
      "message": "Unknown property “anchor-name”.",
      "severity": "Error"
    },
    {
      "message": "Unknown property “anchor-scope”.",
      "severity": "Error"
    }
  ],
  "warnings": 0
}
```

## style[2]

```css

    #test_slot {
      .anchor {
        background: skyblue;
        height: 10px;
        anchor-name: --a;
      }
      .cb {
        position: relative;
        width: 50px;
        height: 50px;
        border: 1px solid black;
      }
      .anchored {
        background: coral;
        position: absolute;
        top: anchor(bottom, 1px);
        position-anchor: --a;
        width: 5px;
        height: 5px;
      }
    }
  
```

```json
{
  "errors": 4,
  "messages": [
    {
      "message": "Invalid value for property “background”.",
      "severity": "Error"
    },
    {
      "message": "Unknown property “anchor-name”.",
      "severity": "Error"
    },
    {
      "message": "Invalid value for property “background”.",
      "severity": "Error"
    },
    {
      "message": "Unknown property “position-anchor”.",
      "severity": "Error"
    }
  ],
  "warnings": 0
}
```

## style[3]

```css

          .scope {
            anchor-scope: --a;
          }
        
```

```json
{
  "errors": 1,
  "messages": [
    {
      "message": "Unknown property “anchor-scope”.",
      "severity": "Error"
    }
  ],
  "warnings": 0
}
```

## style[4]

```css

    #test_slotted {
      .anchor {
        background: skyblue;
        height: 10px;
        anchor-name: --a;
      }
      .cb {
        position: relative;
        width: 50px;
        height: 50px;
        border: 1px solid black;
      }
      .anchored {
        background: coral;
        position: absolute;
        top: anchor(bottom, 1px);
        position-anchor: --a;
        width: 5px;
        height: 5px;
      }
    }
  
```

```json
{
  "errors": 4,
  "messages": [
    {
      "message": "Invalid value for property “background”.",
      "severity": "Error"
    },
    {
      "message": "Unknown property “anchor-name”.",
      "severity": "Error"
    },
    {
      "message": "Invalid value for property “background”.",
      "severity": "Error"
    },
    {
      "message": "Unknown property “position-anchor”.",
      "severity": "Error"
    }
  ],
  "warnings": 0
}
```

## style[5]

```css

          ::slotted(div) {
            anchor-scope: --a;
          }
        
```

```json
{
  "errors": 1,
  "messages": [
    {
      "message": "Unknown property “anchor-scope”.",
      "severity": "Error"
    }
  ],
  "warnings": 0
}
```

## style[6]

```css

    #test_host {
      .anchor {
        background: skyblue;
        height: 10px;
        anchor-name: --a;
      }
      .cb {
        position: relative;
        width: 50px;
        height: 50px;
        border: 1px solid black;
      }
      .anchored {
        background: coral;
        position: absolute;
        top: anchor(bottom, 1px);
        position-anchor: --a;
        width: 5px;
        height: 5px;
      }
    }
  
```

```json
{
  "errors": 4,
  "messages": [
    {
      "message": "Invalid value for property “background”.",
      "severity": "Error"
    },
    {
      "message": "Unknown property “anchor-name”.",
      "severity": "Error"
    },
    {
      "message": "Invalid value for property “background”.",
      "severity": "Error"
    },
    {
      "message": "Unknown property “position-anchor”.",
      "severity": "Error"
    }
  ],
  "warnings": 0
}
```

## style[7]

```css

          :host {
            anchor-scope: --a;
          }
        
```

```json
{
  "errors": 1,
  "messages": [
    {
      "message": "Unknown property “anchor-scope”.",
      "severity": "Error"
    }
  ],
  "warnings": 0
}
```

## style[8]

```css

    #test_part_slotted_in {
      .host::part(scope) {
        anchor-scope: --a;
      }
      .anchor {
        background: skyblue;
        height: 10px;
        anchor-name: --a;
      }
      .cb {
        position: relative;
        width: 50px;
        height: 50px;
        border: 1px solid black;
      }
      .anchored {
        background: coral;
        position: absolute;
        top: anchor(bottom, 1px);
        position-anchor: --a;
        width: 5px;
        height: 5px;
      }
    }
  
```

```json
{
  "errors": 6,
  "messages": [
    {
      "message": "Invalid selector.",
      "severity": "Error"
    },
    {
      "message": "Unknown property “anchor-scope”.",
      "severity": "Error"
    },
    {
      "message": "Invalid value for property “background”.",
      "severity": "Error"
    },
    {
      "message": "Unknown property “anchor-name”.",
      "severity": "Error"
    },
    {
      "message": "Invalid value for property “background”.",
      "severity": "Error"
    },
    {
      "message": "Unknown property “position-anchor”.",
      "severity": "Error"
    }
  ],
  "warnings": 0
}
```
