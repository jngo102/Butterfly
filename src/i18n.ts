import { createI18n } from "vue-i18n"

const messages = {
    en: {
        message: {
            all: "All",
            cancel: "Cancel",
            chooseSaveSlot: "Choose save slot number:",
            createNewProfile: "Create New Profile",
            createProfile: "Create Profile",
            delete: "Delete",
            dependencies: "Dependencies",
            disable: "Disable",
            disableApi: "Disable API",
            donate: "Buy me a coffee",
            enable: "Enable",
            enableApi: "Enable API",
            enabled: "Enabled",
            export: "Export",
            exportProfiles: "Export Profiles",
            importProfiles: "Import Profiles",
            importSave: "Import Save",
            install: "Install",
            installed: "Installed",
            languages: "Languages",
            manualInstall: "Manual Install",
            modReadMe: "ReadMe",
            newMod: "New!",
            openMods: "Open Mods",
            profileNamePlaceholder: "Enter profile name here",
            profileNamePrompt: "Give your profile a name",
            profiles: "Profiles",
            readMe: "Read me",
            report: "Report a bug/suggest a feature",
            reset: "Reset",
            searchMods: "Search Mods",
            selectMods: "Select Mods",
            source: "Source code",
            uninstall: "Uninstall",
            update: "Update",
        },
    },
    cn: {
        message: {
            all: "全部",
            cancel: "取消",
            chooseSaveSlot: "选择Save",
            createNewProfile: "创建新的Mod配置",
            createProfile: "创建Mod配置",
            delete: "删除",
            dependencies: "依赖",
            disable: "禁用",
            disableApi: "禁用API",
            donate: "赞助开发者",
            enableApi: "启用API",
            enable: "启用",
            enabled: "已启用",
            export: "导出",
            exportProfiles: "导出配置文件",
            importProfiles: "导入配置文件",
            importSave: "导入Save",
            install: "安装",
            installed: "已安装",
            languages: "语言",
            manualInstall: "手动安装",
            modReadMe: "自述文件",
            newMod: "",
            openMods: "打开Mods",
            profileNamePlaceholder: "在此处输入Mod配置名称",
            profileNamePrompt: "命名Mod配置",
            profiles: "配置",
            readMe: "自述文件",
            report: "报告BUG/提供建议",
            reset: "重启",
            searchMods: "搜索模组",
            selectMods: "选择模组",
            source: "源代码", 
            uninstall: "卸载",
            update: "更新",
        },
    },
    de: {
        message: {
            all: "",
            cancel: "",
            chooseSaveSlot: "",
            createNewProfile: "",
            createProfile: "",
            delete: "",
            dependencies: "",
            disable: "",
            disableApi: "",
            donate: "",
            enable: "",
            enableApi: "",
            enabled: "",
            export: "",
            exportProfiles: "",
            importProfiles: "",
            importSave: "",
            install: "",
            installed: "",
            languages: "",
            manualInstall: "",
            modReadMe: "",
            newMod: "",
            openMods: "",
            profileNamePlaceholder: "",
            profileNamePrompt: "",
            profiles: "",
            readMe: "",
            report: "",
            reset: "",
            searchMods: "",
            selectMods: "",
            source: "",
            uninstall: "",
            update: "",
        },
    },
    es: {
        message: {
            all: "",
            cancel: "",
            chooseSaveSlot: "",
            createNewProfile: "",
            createProfile: "",
            delete: "",
            dependencies: "",
            disable: "",
            disableApi: "",
            donate: "",
            enable: "",
            enableApi: "",
            enabled: "",
            export: "",
            exportProfiles: "",
            importProfiles: "",
            importSave: "",
            install: "",
            installed: "",
            languages: "",
            manualInstall: "",
            modReadMe: "",
            newMod: "",
            openMods: "",
            profileNamePlaceholder: "",
            profileNamePrompt: "",
            profiles: "",
            readMe: "",
            report: "",
            reset: "",
            searchMods: "",
            selectMods: "",
            source: "",
            uninstall: "",
            update: "",
        },
    },
    fr: {
        message: {
            all: "Tous",
            cancel: "Annuler",
            chooseSaveSlot: "Choisir le numéro de la sauvegarde :",
            createNewProfile: "Créer un nouveau profil",
            createProfile: "Créer un profil",
            delete: "Supprimer",
            dependencies: "Dépendances",
            disable: "Désactiver",
            disableApi: "Désactiver l'API",
            donate: "Faire une donation",
            enable: "Activer",
            enableApi: "Activer l'API",
            enabled: "Activés",
            export: "Exporter",
            exportProfiles: "Exporter les profils",
            importProfiles: "Importer les profils",
            importSave: "Importer une sauvegarde",
            install: "Installer",
            installed: "Installés",
            languages: "Langues",
            manualInstall: "Installation manuelle",
            modReadMe: "README",
            newMod: "Nouveau !",
            openMods: "Ouvrir le dossier des mods",
            profileNamePlaceholder: "Entrer le nom du profil",
            profileNamePrompt: "Donner un nom au profil",
            profiles: "Profils",
            readMe: "README",
            report: "Signaler un bug / suggérer une fonctionnalité",
            reset: "Réinitialiser",
            searchMods: "Chercher des mods",
            selectMods: "Sélectionner des mods",
            source: "Code source",
            uninstall: "Désinstaller",
            update: "Mettre à jour",
        },
    },
    ru: {
        message: {
            all: "",
            cancel: "",
            chooseSaveSlot: "",
            createNewProfile: "",
            createProfile: "",
            delete: "",
            dependencies: "",
            disable: "",
            disableApi: "",
            donate: "",
            enable: "",
            enableApi: "",
            enabled: "",
            export: "",
            exportProfiles: "",
            importProfiles: "",
            importSave: "",
            install: "",
            installed: "",
            languages: "",
            manualInstall: "",
            modReadMe: "",
            newMod: "",
            openMods: "",
            profileNamePlaceholder: "",
            profileNamePrompt: "",
            profiles: "",
            readMe: "",
            report: "",
            reset: "",
            searchMods: "",
            selectMods: "",
            source: "",
            uninstall: "",
            update: "",
        },
    },
};

const i18n = createI18n({
    legacy: false,
    locale: "en",
    globalInjection: true,
    messages: messages,
})

export default i18n;
export const translate = (key: string) => {
    if (!key) {
        return "";
    }

    return i18n.global.t(key);
};
