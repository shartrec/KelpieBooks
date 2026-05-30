<!--
  ~ Copyright (c) 2026.
  ~
  ~ This file is part of KelpieBooks. For terms of use, please see the file
  ~ called LICENSE at the top level of the KelpieBooks source tree
  ~  (online at: https://github.com/shartrec/kelpiebooks/LICENSE ).
  -->

<div class="warning-card">
    <div class="warning-card-icon">⚠️</div>
    <div class="warning-card-content">
        {{ body | markdown(inline=true) | safe }}
    </div>
</div>
