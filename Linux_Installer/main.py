import os
import platform
import shutil
import xml.etree.ElementTree

slafile = "../Linux_XKB/sla"
slaxml = "../Linux_XKB/slaXML"
newbasexml = "XMLsBackup/new_base.xml"
newevdevxml = "XMLsBackup/new_evdev.xml"


# Try to Copy & Edit XML-Files
def installer(target, base, evdev):
    try:
        addXMLEntries(base, evdev)
        shutil.copy(slafile, target)
        shutil.copy(newbasexml, base)
        shutil.copy(newevdevxml, evdev)
    except PermissionError as err:
        print("Permissions denied, you have to use superuser.")
        return False
    return True


# Find layout of a specific name inside the layous-list
def findEntry(layouts, entryName):
    for layout in layouts:
        if layout.find("configItem").find("name").text == entryName:
            return layout


# Get index of "sla" entry in the layouts-list
def getIndex(layouts):
    idx = 0
    for layout in layouts:
        if layout.find("configItem").find("name").text.__eq__("sla"):
            return idx
        else:
            idx = idx + 1
    pass


# Adds Entries to the XML-File
def addXMLEntries(base, evdev):
    xmlsla = xml.etree.ElementTree.parse(slaxml)
    xmlbase = xml.etree.ElementTree.parse(base)
    xmlevdev = xml.etree.ElementTree.parse(evdev)

    if isAleadyIn(xmlbase.getroot().find("layoutList").findall("layout"), "sla"):
        index = getIndex(xmlbase.getroot().find("layoutList").findall("layout"))
        xmlbase.getroot().find("layoutList").remove(xmlbase.getroot().find("layoutList").__getitem__(index))

    xmlbase.getroot().find("layoutList").append(xmlsla.getroot())
    xmlbase.write(newbasexml, encoding='unicode')

    if isAleadyIn(xmlevdev.getroot().find("layoutList").findall("layout"), "sla"):
        index = getIndex(xmlevdev.getroot().find("layoutList").findall("layout"))
        xmlevdev.getroot().find("layoutList").remove(xmlevdev.getroot().find("layoutList").__getitem__(index))

    xmlevdev.getroot().find("layoutList").append(xmlsla.getroot())
    xmlevdev.write(newevdevxml, encoding='unicode')
    pass


# Check if the layout is in the layouts-list
def isAleadyIn(layouts, layoutNameToFind):
    for layout in layouts:
        if findLayoutName(layout, layoutNameToFind):
            return True
    return False


# Check if the layout's name is same as the searched for
def findLayoutName(layout, layoutNameToFind):
    configitem = layout.find("configItem")
    name = configitem.find("name")
    nametext = name.text
    return nametext == layoutNameToFind


# Check whether the provided entries are properly XML-Files and whether the old XML files had a entry already in
def checkXMLEntries(base, evdev):
    if not findLayoutName(xml.etree.ElementTree.parse(slaxml).getroot(), "sla"):
        print("XML-File corrupted.")
        return False

    if isAleadyIn(xml.etree.ElementTree.parse(base).getroot().find("layoutList").findall("layout"), "sla") | isAleadyIn(
            xml.etree.ElementTree.parse(evdev).getroot().find("layoutList").findall("layout"), "sla"):
        print("XML-Entries already in, old entries will be overwritten.")
    return True


# Recheck whether the distribution-provided files are in-place & make backups of the current one's
def checkOSFiles(target, base, evdev):
    if os.path.exists(target) & os.path.exists(base) & os.path.exists(evdev):
        if os.path.exists(target + "sla"):
            os.remove(target + "sla")

            if not os.path.exists("./XMLsBackup/base"):
                shutil.copy(base, "./XMLsBackup/")
            else:
                # TODO optional override or not parser
                print("The Old Backup-files will be not overridden, please backup your old-backup-files manually")
                return False

            if not os.path.exists("./XMLsBackup/evdev"):
                shutil.copy(evdev, "./XMLsBackup/")
            else:
                # TODO optional override or not parser
                print("The Old Backup-files will be not overridden, please backup your old-backup-files manually")
                return False

        return checkXMLEntries(base, evdev)
    print("Distribution or its version unknown!")
    return False


# Determines distribution and set's environment variables
def selector(distribution):
    target, base, evdev = "", "", ""

    # TODO check distribution-depending paths and save them
    if "Debian" in distribution:
        print(distribution)
    elif "Ubuntu" in distribution:
        print(distribution)
    elif "Fedora" in distribution:
        print(distribution)
    elif "Slackware" in distribution:
        print(distribution)
    elif ("Arch" in distribution) | ("MANJARO" in distribution):
        print("Arch / Manjaro Linux detected")
        target = "/usr/share/X11/xkb/symbols"
        base = "/usr/share/X11/xkb/rules/base.xml"
        evdev = "/usr/share/X11/xkb/rules/evdev.xml"
    elif "Gentoo" in distribution:
        print(distribution)
    elif "Puppy" in distribution:
        print(distribution)
    else:
        print("Distribution could not be identified!")
        return False

    if not checkOSFiles(target, base, evdev):
        return False
    return installer(target, base, evdev)


# Checks whether the prerequisites are in place
def checkIntegrity():
    if os.path.exists(slafile):
        return True
    if os.path.exists(slaxml):
        return True
    print("Archive corrupted!")
    return False


def main():

    if not platform.system().__eq__("Linux"):
        print("Unsupported System: " + platform.system())
    else:
        if checkIntegrity():
            if selector(platform.release()).__eq__(True):
                print("### Installed successfully ###")
                return
            print("### Installation canceled ###")


if __name__ == '__main__':
    main()
