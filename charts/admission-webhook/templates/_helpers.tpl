{{/*
Expand the name of the chart.
*/}}
{{- define "admission-webhook.name" -}}
{{- default .Chart.Name .Values.nameOverride | trunc 63 | trimSuffix "-" }}
{{- end }}

{{/*
Create a default fully qualified app name.
We truncate at 63 chars because some Kubernetes name fields are limited to this (by the DNS naming spec).
If release name contains chart name it will be used as a full name.
*/}}
{{- define "admission-webhook.fullname" -}}
{{- if .Values.fullnameOverride }}
{{- .Values.fullnameOverride | trunc 63 | trimSuffix "-" }}
{{- else }}
{{- $name := default .Chart.Name .Values.nameOverride }}
{{- if contains $name .Release.Name }}
{{- .Release.Name | trunc 63 | trimSuffix "-" }}
{{- else }}
{{- printf "%s-%s" .Release.Name $name | trunc 63 | trimSuffix "-" }}
{{- end }}
{{- end }}
{{- end }}

{{/*
Create chart name and version as used by the chart label.
*/}}
{{- define "admission-webhook.chart" -}}
{{- printf "%s-%s" .Chart.Name .Chart.Version | replace "+" "_" | trunc 63 | trimSuffix "-" }}
{{- end }}

{{- define "admission-webhook.version" -}}
{{ default .Chart.AppVersion }}
{{- end }}

{{/*
Common labels
*/}}
{{- define "admission-webhook.labels" -}}
helm.sh/chart: {{ include "admission-webhook.chart" . }}
{{ include "admission-webhook.selectorLabels" . }}
{{- if .Chart.AppVersion }}
app.kubernetes.io/version: {{ .Chart.AppVersion | quote }}
{{- end }}
app.kubernetes.io/managed-by: {{ .Release.Service }}
{{- end }}

{{/*
Selector labels
*/}}
{{- define "admission-webhook.selectorLabels" -}}
app.kubernetes.io/name: {{ include "admission-webhook.name" . }}
app.kubernetes.io/instance: {{ .Release.Name }}
{{- end }}

{{- define "admission-webhook.podInfoEnvs" -}}
- name: POD_NAMESPACE
  valueFrom:
    fieldRef:
      apiVersion: v1
      fieldPath: metadata.namespace
- name: POD_NAME
  valueFrom:
    fieldRef:
      fieldPath: metadata.name
- name: POD_IP
  valueFrom:
    fieldRef:
      apiVersion: v1
      fieldPath: status.podIP
- name: NODE_NAME
  valueFrom:
    fieldRef:
      apiVersion: v1
      fieldPath: spec.nodeName
- name: ISTIO_REV
  valueFrom:
    fieldRef:
      apiVersion: v1
      fieldPath: metadata.labels['istio.io/rev']
{{- end }}

{{- define "admission-webhook.domain" -}}
{{- printf "%s.svc.%s" .Release.Namespace .Values.clusterDomain }}
{{- end }}

{{- define "admission-webhook.certificateName" -}}
{{- printf "%s/%s" .Release.Namespace .Values.certificate.name }}
{{- end }}