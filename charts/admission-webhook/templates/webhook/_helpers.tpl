{{/* vim: set ft=mustache: */}}
{{/*
Expand the name of the chart.
*/}}
{{- define "admission-webhook.webhook.name" -}}
{{ include "admission-webhook.name" . }}
{{- end }}

{{/*
Create a default fully qualified app name.
We truncate at 63 chars because some Kubernetes name fields are limited to this (by the DNS naming spec).
If release name contains chart name it will be used as a full name.
*/}}
{{- define "admission-webhook.webhook.fullname" -}}
{{ include "admission-webhook.fullname" . }}
{{- end }}

{{/*
Common labels
*/}}
{{- define "admission-webhook.webhook.labels" -}}
{{ include "admission-webhook.labels" . }}
app.kubernetes.io/component: webhook
{{- end }}

{{/*
Selector labels
*/}}
{{- define "admission-webhook.webhook.selectorLabels" -}}
{{ include "admission-webhook.selectorLabels" . }}
app.kubernetes.io/component: webhook
{{- end }}

{{/*
Create the name of the service account to use
*/}}
{{- define "admission-webhook.webhook.serviceAccountName" -}}
{{- if .Values.webhook.serviceAccount.create }}
{{- default (include "admission-webhook.webhook.fullname" .) .Values.webhook.serviceAccount.name }}
{{- else }}
{{- default "default" .Values.webhook.serviceAccount.name }}
{{- end }}
{{- end }}

{{- if gt .Values.webhook.replicas 1 }}
{{- fail "HA is not currently support" }}
{{- end }}

{{- define "admission-webhook.webhook.logLevel" -}}
{{- default .Values.logLevel .Values.webhook.logLevel }}
{{- end }}

{{- define "admission-webhook.webhook.image.registry" -}}
{{- default .Values.webhook.image.registry }}
{{- end }}

{{- define "admission-webhook.webhook.image.tag" -}}
{{- default .Values.webhook.image.tag }}
{{- end }}

{{- define "admission-webhook.webhook.host" -}}
{{- printf "%s.%s" (include "admission-webhook.webhook.fullname" .) (include "admission-webhook.domain" .) }}
{{- end }}

{{- define "admission-webhook.webhook.httpEndpoint" -}}
{{- printf "http://%s:%s" (include "admission-webhook.webhook.host" .) (include "admission-webhook.webhook.httpPort" .) }}
{{- end }}

{{- define "admission-webhook.webhook.httpPort" -}}
8007
{{- end }}

{{- define "admission-webhook.webhook.metricsPort" -}}
8200
{{- end }}

{{- define "admission-webhook.webhook.metricsEndpoint" -}}
{{- printf "http://%s.%s:%s" (include "admission-webhook.webhook.fullname" .) (include "admission-webhook.domain" .) (include "admission-webhook.webhook.metricsPort" .) }}
{{- end }}